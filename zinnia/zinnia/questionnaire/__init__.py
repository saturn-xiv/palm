from flask import (Blueprint, render_template)

router = Blueprint('questionnaire', __name__, url_prefix='/questionnaire')


@router.route('/forms/<token>', methods=['GET'])
def show_form(token):
    return render_template('questionnaire/forms/show.html', token=token)
